<script setup>
import { ref } from "vue";

let props = defineProps({
    confirm: Function,
    cancel: Function
})

const errMsg = ref("");
const filename = ref("");
const handleValChange = () => {
    if(!filename.value.trim()) {
        errMsg.value = "不能为空";
        return
    }
    props.confirm(filename.value);
    filename.value = "";
}
const cancel = () => {
    filename.value = "";
    props.cancel();
}
</script>

<template>
    <div class="dialog">
        <div class="form">
            <label>文件名:</label>
            <input autofocus v-model="filename" @change="handleValChange" />
        </div>
        <p class="error" v-if="errMsg">ERROR:{{errMsg}}</p>
        <div class="btns">
            <button @click="cancel">取消</button>
            <button @click="handleValChange">确认</button>
        </div>
    </div>
</template>

<style scoped>
p {
    margin: 0;
    padding: 0;
}
.dialog {
    position: fixed;
    top: 40vh;
    left: 40vw;
    padding: 40px 30px 20px;
    display: flex;
    flex-direction: column;
    background-color: lightblue;
    border-radius: 15px;
    z-index: 9999;
}

label {
    margin-right: 20px;
}

.form {
    margin-bottom: 20px;
}

.btns {
    text-align: right;
}

.btns button {
    padding: 6px 12px;
    cursor: pointer;
    color: #fff;
    background-color: #409eff;
    border: none;
    outline: none;
    border-radius: 6px;
}

button:first-child {
    background-color: #f56c6c;
}

.error {
    color: red;
    font-size: 12px;
    font-weight: bold;
}
</style>