export const IpFieldService = {
    getData() {
        return [
            {
                key: "地址类型",
                field: "fieldType",
                value: "",
            },
            {
                key: "压缩地址",
                field: "comAddress",
                value: "",
            },
            {
                key: "扩展地址",
                field: "exAddress",
                value: "",
            },
            {
                key: "二进制地址",
                field: "binaryAddress",
                value: "",
            },
            {
                key: "子网",
                field: "subnet",
                value: "",
            },
            {
                key: "子网掩码",
                field: "subnetMask",
                value: "",
            },
            {
                key: "上一个地址",
                field: "prevAddress",
                value: "",
            },
            {
                key: "下一个地址",
                field: "nextAddress",
                value: "",
            },
            {
                key: "整数值",
                field: "intValue",
                value: "",
            },
            {
                key: "高低64位有符号数",
                field: "highLow64BitSignedNumber",
                value: "",
            },
            {
                key: "转为 IPv4",
                field: "toIpv4",
                value: "",
            },
            {
                key: "转为 IPv6",
                field: "toIpv6",
                value: "",
            },
            {
                key: "网络地址（开始ip）",
                field: "netWorkAddress",
                value: "",
            },
            {
                key: "网络地址（开始ip）整数值",
                field: "netWorkAddressIntValue",
                value: "",
            },
            {
                key: "网络地址（开始ip）高低64位有符号数",
                field: "netWorkAddressHighLow64BitSignedNumber",
                value: "",
            },
            {
                key: "网络地址（开始ip）二进制地址",
                field: "netWorkAddressBinaryAddress",
                value: "",
            },
            {
                key: "广播地址（结束ip）",
                field: "broadcastAddress",
                value: "",
            },
            {
                key: "广播地址（结束ip）整数值",
                field: "broadcastAddressIntValue",
                value: "",
            },
            {
                key: "广播地址（结束ip）高低64位有符号数",
                field: "broadcastAddressHighLow64BitSignedNumber",
                value: "",
            },
            {
                key: "广播地址（结束ip）二进制地址",
                field: "broadcastAddressBinaryAddress",
                value: "",
            }
        ];
    },

    getFieldList() {
        return this.getData();
    }
};
