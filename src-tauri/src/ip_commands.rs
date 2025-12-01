use serde::Serialize;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[derive(Serialize, Debug)]
pub enum IpTranslationType {
    Ipv4,
    Ipv6,
    Ipv4Mask,
    Ipv6Mask,
    Ipv4Range,
    Ipv6Range,
    Ipv4Num,
    Ipv6Num,
    UnknownIp,
}

impl IpTranslationType {
    pub fn from_str(input: &str) -> Self {
        let input = input.trim();

        // 1. 范围：a-b
        if input.contains('-') {
            let parts: Vec<_> = input.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (Ipv4Addr::from_str(parts[0]), Ipv4Addr::from_str(parts[1])) {
                    // 验证是否真的是范围（start < end）
                    let start_int = u32::from_be_bytes(start.octets()) as u128;
                    let end_int = u32::from_be_bytes(end.octets()) as u128;
                    if start_int < end_int {
                        return Self::Ipv4Range;
                    }
                }
                if let (Ok(start), Ok(end)) = (Ipv6Addr::from_str(parts[0]), Ipv6Addr::from_str(parts[1])) {
                    // 验证是否真的是范围（start < end）
                    let start_int = u128::from_be_bytes(start.octets());
                    let end_int = u128::from_be_bytes(end.octets());
                    if start_int < end_int {
                        return Self::Ipv6Range;
                    }
                }
            }
        }

        // 2. 掩码 CIDR 输入
        if input.contains('/') {
            let parts: Vec<_> = input.split('/').collect();
            if parts.len() == 2 {
                if let Ok(ipv4) = Ipv4Addr::from_str(parts[0]) {
                    if let Ok(mask) = parts[1].parse::<u8>() {
                        if mask <= 32 {
                            return Self::Ipv4Mask;
                        }
                    }
                }
                if let Ok(ipv6) = Ipv6Addr::from_str(parts[0]) {
                    if let Ok(mask) = parts[1].parse::<u8>() {
                        if mask <= 128 {
                            return Self::Ipv6Mask;
                        }
                    }
                }
            }
        }

        // 3. 直接 IPv4
        if let Ok(_ipv4) = Ipv4Addr::from_str(input) {
            return Self::Ipv4;
        }

        // 4. 直接 IPv6
        if let Ok(_ipv6) = Ipv6Addr::from_str(input) {
            return Self::Ipv6;
        }

        // 5. 数字类型 IPv4Num 或 IPv6Num
        if let Ok(num) = input.parse::<u128>() {
            if num <= u32::MAX as u128 {
                return Self::Ipv4Num;
            } else if num <= u128::MAX {
                return Self::Ipv6Num;
            }
        }

        Self::UnknownIp
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IpTranslationResult {
    pub field_type: String,
    pub com_address: String,
    pub ex_address: String,
    pub binary_address: String,
    pub subnet: String,
    pub subnet_mask: String,
    pub prev_address: String,
    pub next_address: String,
    pub int_value: String, // 用字符串表示 u128，避免 JS 精度问题
    pub high_low_64_bit_signed_number: Option<(i64, i64)>,
    pub to_ipv4: String,
    pub to_ipv6: String,
    pub net_work_address: String,
    pub net_work_address_int_value: String,
    pub net_work_address_binary_address: String,
    pub broadcast_address: String,
    pub broadcast_address_int_value: String,
    pub broadcast_address_binary_address: String,
}

impl Default for IpTranslationResult {
    fn default() -> Self {
        Self {
            field_type: "".into(),
            com_address: "".into(),
            ex_address: "".into(),
            binary_address: "".into(),
            subnet: "".into(),
            subnet_mask: "".into(),
            prev_address: "".into(),
            next_address: "".into(),
            int_value: "".into(),
            high_low_64_bit_signed_number: None,
            to_ipv4: "".into(),
            to_ipv6: "".into(),
            net_work_address: "".into(),
            net_work_address_int_value: "".into(),
            net_work_address_binary_address: "".into(),
            broadcast_address: "".into(),
            broadcast_address_int_value: "".into(),
            broadcast_address_binary_address: "".into(),
        }
    }
}

fn ipv4_to_u32(ip: &Ipv4Addr) -> u32 {
    u32::from_be_bytes(ip.octets())
}

fn u32_to_ipv4(n: u32) -> Ipv4Addr {
    Ipv4Addr::from(n)
}

fn format_ipv4_binary(ip: &Ipv4Addr) -> String {
    ip.octets()
        .iter()
        .map(|oct| format!("{:08b}", oct))
        .collect::<Vec<String>>()
        .join(".")
}

fn split_u128_to_i64(num: u128) -> (i64, i64) {
    let high = (num >> 64) as i64;
    let low = (num & 0xFFFFFFFFFFFFFFFF) as i64;
    (high, low)
}

fn ipv6_to_u128(ip: &Ipv6Addr) -> u128 {
    let segments = ip.segments();
    segments
        .iter()
        .fold(0u128, |acc, &seg| (acc << 16) | seg as u128)
}

fn u128_to_ipv6(num: u128) -> Ipv6Addr {
    let segments = [
        ((num >> 112) & 0xFFFF) as u16,
        ((num >> 96) & 0xFFFF) as u16,
        ((num >> 80) & 0xFFFF) as u16,
        ((num >> 64) & 0xFFFF) as u16,
        ((num >> 48) & 0xFFFF) as u16,
        ((num >> 32) & 0xFFFF) as u16,
        ((num >> 16) & 0xFFFF) as u16,
        (num & 0xFFFF) as u16,
    ];
    Ipv6Addr::new(
        segments[0],
        segments[1],
        segments[2],
        segments[3],
        segments[4],
        segments[5],
        segments[6],
        segments[7],
    )
}

fn format_ipv6_binary(ip: &Ipv6Addr) -> String {
    let segments = ip.segments();
    let binary_parts: Vec<String> = segments
        .iter()
        .map(|&seg| format!("{:016b}", seg))
        .collect();
    
    let upper_half: String = binary_parts[..4]
        .join("")
        .as_bytes()
        .chunks(16)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<&str>>()
        .join(":");
        
    let lower_half: String = binary_parts[4..]
        .join("")
        .as_bytes()
        .chunks(16)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<&str>>()
        .join(":");
        
    format!("{}\n{}", upper_half, lower_half)
}

#[tauri::command]
pub fn translate_ip(ip: String) -> Result<IpTranslationResult, String> {
    let ip_type = IpTranslationType::from_str(&ip);
    let mut result = IpTranslationResult::default();
    
    match ip_type {
        IpTranslationType::Ipv4 => {
            let ipv4 = Ipv4Addr::from_str(&ip).map_err(|e| format!("无效的 IPv4 地址 '{}': {}", ip, e))?;
            let int_value = ipv4_to_u32(&ipv4) as u128;
            result.field_type = "IPv4".into();
            result.com_address = ipv4.to_string();
            result.binary_address = format_ipv4_binary(&ipv4);
            // 上一个地址
            if int_value > 0 {
                result.prev_address = u32_to_ipv4((int_value - 1) as u32).to_string();
            }
            // 下一个地址
            if int_value < u32::MAX as u128 {
                result.next_address = u32_to_ipv4((int_value + 1) as u32).to_string();
            }
            result.int_value = int_value.to_string();
            result.to_ipv4 = ipv4.to_string();
            result.to_ipv6 = format!("0:0:0:0:0:ffff:{}", ipv4);
            result.high_low_64_bit_signed_number = Some(split_u128_to_i64(int_value));
        }
        IpTranslationType::Ipv6 => {
            let ipv6 = Ipv6Addr::from_str(&ip).map_err(|e| format!("无效的 IPv6 地址 '{}': {}", ip, e))?;
            let int_value = ipv6_to_u128(&ipv6);

            result.field_type = "IPv6".into();
            result.com_address = ipv6.to_string(); // 压缩形式
            result.ex_address = ipv6
                .segments()
                .iter()
                .map(|seg| format!("{:04x}", seg))
                .collect::<Vec<String>>()
                .join(":"); // 完整展开形式

            result.binary_address = format_ipv6_binary(&ipv6);
            result.int_value = int_value.to_string();
            result.high_low_64_bit_signed_number = Some(split_u128_to_i64(int_value));

            // 上一个地址
            if int_value > 0 {
                result.prev_address = u128_to_ipv6(int_value - 1).to_string();
            }

            // 下一个地址
            if int_value < u128::MAX {
                result.next_address = u128_to_ipv6(int_value + 1).to_string();
            }
            result.to_ipv6 = ipv6.to_string();
        }
        IpTranslationType::Ipv4Mask => {
            let parts: Vec<&str> = ip.split('/').collect();
            if parts.len() != 2 {
                return Err(format!("无效的 IPv4 CIDR 格式 '{}'", ip));
            }
            
            let ipv4 = Ipv4Addr::from_str(parts[0]).map_err(|e| format!("无效的 IPv4 地址 '{}': {}", parts[0], e))?;
            let mask_len: u8 = parts[1].parse().map_err(|e| format!("无效的掩码长度 '{}': {}", parts[1], e))?;
            
            // 计算子网掩码
            let mask: u32 = if mask_len == 0 {
                0
            } else {
                (!0u32) << (32 - mask_len)
            };
            let ip_u32 = ipv4_to_u32(&ipv4);
            let network = ip_u32 & mask;
            let broadcast = network | !mask;

            result.field_type = "IPv4 + CIDR 掩码".into();
            result.com_address = ipv4.to_string();
            result.subnet = format!("{}/{}", ipv4, mask_len);
            result.subnet_mask = u32_to_ipv4(mask).to_string();
            result.net_work_address = u32_to_ipv4(network).to_string();
            result.broadcast_address = u32_to_ipv4(broadcast).to_string();
            result.int_value = ip_u32.to_string();
            result.high_low_64_bit_signed_number = Some(split_u128_to_i64(ip_u32 as u128));
            result.binary_address = format_ipv4_binary(&ipv4);
            result.net_work_address_int_value = network.to_string();
            result.broadcast_address_int_value = broadcast.to_string();
            result.net_work_address_binary_address = format_ipv4_binary(&u32_to_ipv4(network));
            result.broadcast_address_binary_address = format_ipv4_binary(&u32_to_ipv4(broadcast));
            result.to_ipv4 = ipv4.to_string();
            result.to_ipv6 = format!("0:0:0:0:0:ffff:{}", ipv4);
        }
        IpTranslationType::Ipv6Mask => {
            let parts: Vec<&str> = ip.split('/').collect();
            if parts.len() != 2 {
                return Err(format!("无效的 IPv6 CIDR 格式 '{}'", ip));
            }
            
            let ipv6 = Ipv6Addr::from_str(parts[0]).map_err(|e| format!("无效的 IPv6 地址 '{}': {}", parts[0], e))?;
            let mask_len: u8 = parts[1].parse().map_err(|e| format!("无效的掩码长度 '{}': {}", parts[1], e))?;
            
            if mask_len > 128 {
                return Err(format!("IPv6 掩码长度 '{}' 不能超过 128", mask_len));
            }

            // 转换为整数
            let ipv6_int = ipv6_to_u128(&ipv6);

            // 生成掩码
            let mask: u128 = if mask_len == 0 {
                0
            } else {
                (!0u128) << (128 - mask_len)
            };

            // 计算网络地址
            let network = ipv6_int & mask;
            // 计算子网中最大 IPv6 地址（模拟 IPv4 广播地址逻辑）
            let broadcast = network | (!mask);

            result.field_type = "IPv6 + CIDR 掩码".into();
            result.com_address = ipv6.to_string();
            result.ex_address = ipv6
                .segments()
                .iter()
                .map(|seg| format!("{:04x}", seg))
                .collect::<Vec<String>>()
                .join(":");

            result.subnet = format!("{}/{}", ipv6, mask_len);
            result.subnet_mask = format!("{:x}", mask); // IPv6一般用CIDR，不展示传统掩码
            result.net_work_address = u128_to_ipv6(network).to_string();
            result.net_work_address_int_value = network.to_string();
            result.net_work_address_binary_address = format_ipv6_binary(&u128_to_ipv6(network));

            // IPv6 没有广播地址，这里给空字符串或 "::"
            result.broadcast_address = u128_to_ipv6(broadcast).to_string();
            result.broadcast_address_int_value = broadcast.to_string();
            result.broadcast_address_binary_address = format_ipv6_binary(&u128_to_ipv6(broadcast));

            result.binary_address = format_ipv6_binary(&ipv6);
            result.int_value = ipv6_int.to_string();
            result.high_low_64_bit_signed_number = Some(split_u128_to_i64(ipv6_int));
            result.to_ipv6 = ipv6.to_string();
            result.to_ipv4 = "".into(); // IPv6Mask无法转IPv4
        }
        IpTranslationType::Ipv4Range => {
            // 解析 a.b.c.d-e.f.g.h
            let parts: Vec<&str> = ip.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("无效的 IPv4 范围格式 '{}'", ip));
            }

            let start_ip = Ipv4Addr::from_str(parts[0].trim()).map_err(|e| format!("起始 IPv4 地址 '{}' 无效: {}", parts[0], e))?;
            let end_ip = Ipv4Addr::from_str(parts[1].trim()).map_err(|e| format!("结束 IPv4 地址 '{}' 无效: {}", parts[1], e))?;

            let start_int = ipv4_to_u32(&start_ip) as u128;
            let end_int = ipv4_to_u32(&end_ip) as u128;

            if start_int > end_int {
                return Err(format!("IPv4 范围起始地址 '{}' 不能大于结束地址 '{}'", start_ip, end_ip));
            }

            result.field_type = "IPv4 范围".into();
            result.com_address = format!("{} - {}", start_ip, end_ip);
            result.binary_address = format!(
                "{} - {}",
                format_ipv4_binary(&start_ip),
                format_ipv4_binary(&end_ip)
            );

            // 范围不需要 prev/next 单独处理
            result.net_work_address = start_ip.to_string();
            result.net_work_address_int_value = start_int.to_string();
            result.net_work_address_binary_address = format_ipv4_binary(&start_ip);

            result.broadcast_address = end_ip.to_string();
            result.broadcast_address_int_value = end_int.to_string();
            result.broadcast_address_binary_address = format_ipv4_binary(&end_ip);
        }
        IpTranslationType::Ipv6Range => {
            // 解析 a:b:c::d - a:b:c::e
            let parts: Vec<&str> = ip.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("无效的 IPv6 范围格式 '{}'", ip));
            }

            let start_ip = Ipv6Addr::from_str(parts[0].trim()).map_err(|e| format!("起始 IPv6 地址 '{}' 无效: {}", parts[0], e))?;
            let end_ip = Ipv6Addr::from_str(parts[1].trim()).map_err(|e| format!("结束 IPv6 地址 '{}' 无效: {}", parts[1], e))?;

            let start_int = ipv6_to_u128(&start_ip);
            let end_int = ipv6_to_u128(&end_ip);

            if start_int > end_int {
                return Err(format!("IPv6 范围起始地址 '{}' 不能大于结束地址 '{}'", start_ip, end_ip));
            }

            result.field_type = "IPv6 范围".into();
            // 范围逻辑：开始地址/结束地址
            result.net_work_address = start_ip.to_string();
            result.net_work_address_int_value = start_int.to_string();
            result.net_work_address_binary_address = format_ipv6_binary(&start_ip);

            result.broadcast_address = end_ip.to_string();
            result.broadcast_address_int_value = end_int.to_string();
            result.broadcast_address_binary_address = format_ipv6_binary(&end_ip);
            
            result.com_address = format!("{} - {}", start_ip, end_ip);
            result.binary_address = format!(
                "{} - {}",
                format_ipv6_binary(&start_ip),
                format_ipv6_binary(&end_ip)
            );
        }
        IpTranslationType::Ipv4Num => {
            let num = ip.parse::<u128>().map_err(|e| format!("无效的 IPv4 数字 '{}': {}", ip, e))?;
            if num > u32::MAX as u128 {
                return Err(format!("IPv4 数值 '{}' 必须在 0 ~ 2^32-1 之间", num));
            }
            let ipv4 = u32_to_ipv4(num as u32);
            
            result.field_type = "整数 -> IPv4".into();
            result.com_address = ipv4.to_string();
            result.binary_address = format_ipv4_binary(&ipv4);
            result.int_value = num.to_string();
            // 上一个地址
            if num > 0 {
                result.prev_address = u32_to_ipv4((num - 1) as u32).to_string();
            }
            // 下一个地址
            if num < u32::MAX as u128 {
                result.next_address = u32_to_ipv4((num + 1) as u32).to_string();
            }
            // IPv4 转 IPv6（映射地址）
            result.to_ipv4 = ipv4.to_string();
            result.to_ipv6 = format!("::ffff:{}", ipv4);
            result.high_low_64_bit_signed_number = Some(split_u128_to_i64(num));
        }
        IpTranslationType::Ipv6Num => {
            let num = ip.parse::<u128>().map_err(|e| format!("无效的 IPv6 数字 '{}': {}", ip, e))?;
            let ipv6 = u128_to_ipv6(num);

            result.field_type = "整数 -> IPv6".into();
            result.com_address = ipv6.to_string();
            result.ex_address = ipv6
                .segments()
                .iter()
                .map(|seg| format!("{:04x}", seg))
                .collect::<Vec<String>>()
                .join(":");
            result.binary_address = format_ipv6_binary(&ipv6);
            result.int_value = num.to_string();
            result.high_low_64_bit_signed_number = Some(split_u128_to_i64(num));
            // 上下地址
            if num > 0 {
                result.prev_address = u128_to_ipv6(num - 1).to_string();
            }
            if num < u128::MAX {
                result.next_address = u128_to_ipv6(num + 1).to_string();
            }
            result.to_ipv6 = ipv6.to_string();
        }
        IpTranslationType::UnknownIp => {
            return Err(format!("无法识别 IP 格式 '{}'", ip));
        }
    }
    Ok(result)
}