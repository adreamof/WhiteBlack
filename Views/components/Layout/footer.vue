
<template>
    <div id="footer-content">
        <p>{{ text }}</p>
    </div>
</template>

<script setup>
defineProps(['text'])
</script>

<style scoped>
#footer-content {
    height: 20%;
}


</style>