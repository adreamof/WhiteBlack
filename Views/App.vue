<script setup>
import { ref, onMounted } from "vue";
import { useQuasar } from 'quasar'
import { invoke } from "@tauri-apps/api/tauri";

const $Quasar = useQuasar();
$Quasar.dark.set(false);

const SelectedTab = ref('TopPage');
var msg = ref('');
async function get_task() {
    return invoke("get_task").then((res) => {
        console.log(Object.values(JSON.parse(res)));
        msg.value = JSON.stringify(JSON.parse(res), null, '\t');
    });
}

function send_email_click() {
    return invoke("send_email_click").then((res) => {
        console.log(res);
    })
}

onMounted(() => {

});
</script>

<template>
    <div>
        <q-layout view="hhh lpr fff">
            <q-header elevated>
                <q-toolbar class="justify-around">
                    <q-btn flat round dense icon="Menu" class="q-mr-sm" />
                    <q-tabs v-model="SelectedTab">
                        <q-tab name="TopPage" label="首页" />
                        <q-tab name="ServerPage" label="服务器" />
                        <q-tab name="DownloadPage" label="下载" />
                        <q-tab name="ConfigPage" label="全局设置" />
                    </q-tabs>
                    <q-btn class="col-2" color="black" label="暗色模式" @click="$Quasar.dark.toggle()" />
                </q-toolbar>
            </q-header>

            <q-page-container>
                <q-page class="q-pa-xs">
                    <q-card class="my-card" flat bordered>
                        <q-card-section horizontal>
                            <q-card-section class="q-pt-xs">
                                <div class="text-overline">发送邮件</div>
                                <q-btn class="col-2 q-mt-sm q-mb-xs" color="blue" label="发送邮件" @click="send_email_click" />
                                <div class="text-caption text-grey">
                                    测试内容: Lorem ipsum dolor sit amet consectetur adipisicing elit. Fugit nihil praesentium
                                    molestias a
                                    adipisci, dolore vitae odit, quidem consequatur optio voluptates asperiores pariatur
                                    eos numquam
                                    rerum delectus commodi perferendis voluptate?测试测试测试测试测试
                                </div>
                            </q-card-section>

                            <q-card-section class="col-5 flex-center">
                                <p class="text-grey">游戏版本: 1.19.2</p>
                                <p class="text-caption text-grey">Java版本: JVM 17</p>
                                <p class="text-caption text-grey">游戏路径: C:\Test\Minecraft\version\test</p>
                            </q-card-section>
                        </q-card-section>

                        <q-separator />

                        <q-card-actions>
                            <q-btn flat color="primary">
                                启动游戏
                            </q-btn>
                            <q-btn flat>
                                打开路径
                            </q-btn>
                            <q-btn flat>
                                独立设置
                            </q-btn>
                        </q-card-actions>
                    </q-card>
                </q-page>
            </q-page-container>

            <q-footer elevated reveal>
                <div class="row flex-center text-center q-gutter-sm">
                    <div class="col-2">first</div>
                    <div class="col-2 ellipsis">second..asdsssssadas</div>
                    <div class="col-2">third</div>
                </div>
            </q-footer>
        </q-layout>

    </div>
</template>

<style scoped>
.row {
    background: rgba(255, 0, 0, .1);
}

.row>div {
    background: rgba(86, 61, 124, .15);
    border: 1px solid rgba(86, 61, 124, .2);
}
</style>
