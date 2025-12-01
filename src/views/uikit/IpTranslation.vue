<script setup lang="ts">
import {ref} from 'vue';
import {invoke} from '@tauri-apps/api/core';
import {useToast} from "primevue/usetoast";
import {IpFieldService} from '@/service/IpFieldService';
import {writeText} from '@tauri-apps/plugin-clipboard-manager';

const toast = useToast();
let resultList = ref([]);
let inputIp = ref<string>('172.17.205.22');

const handleResult = () => {
  resultList.value = []
  inputIp.value = inputIp.value.trim()
  invoke<string>('translate_ip', {ip: inputIp.value}).then((data) => {
    let ipFieldList = IpFieldService.getFieldList();
    resultList.value = ipFieldList.map(item => ({
      ...item,
      value: data[item.field] || ''
    })).filter(item => item.value !== '');

  }).catch((err) => {
    toast.add({severity: 'error', summary: '解析失败', detail: err, life: 1300});
  })
}

const copyValue = (value) => {
  writeText(value).then(() => {
    toast.add({severity: 'success', summary: '复制成功', detail: value, life: 1300});
  });
}
</script>

<template>
  <div class="flex flex-col">
    <Toast/>
    <div class="card">
      <div class="font-semibold text-xl mb-4">Ip Translation</div>
      <InputGroup class="w-full">
        <InputText placeholder="输入一个Ipv4地址/Ipv6地址/整数" v-model="inputIp" @keyup.enter="handleResult"/>
        <Button label="解析" @click="handleResult"/>
      </InputGroup>
      <Divider/>
      <DataTable :value="resultList" resizableColumns columnResizeMode="fit" showGridlines>
        <Column field="key" header="字段含义" style="width: 30%;"/>
        <Column field="value" header="字段值" style="width: 70%;">
          <template #body="{ data }">
            <div class="cursor-pointer hover:bg-gray-100 transition-colors relative" @click="copyValue(data.value)">
              <div class="whitespace-pre-line break-all">
                {{ data.value }}
              </div>
            </div>
          </template>
        </Column>
      </DataTable>
    </div>
  </div>
</template>

<style scoped lang="scss">
</style>