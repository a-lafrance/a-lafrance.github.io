<template>
  <ul class="list">
    <li class="list-item" v-for="(item, index) in items" :key="index">
      {{ item }}
    </li>
  </ul>
</template>

<script>
export default {
  name: 'List',
  props: {
    items: Array
  }
}
</script>

<style scoped>
.entry-list {
  padding-left: 2vh;
}

.list-item {
  font-size: 11pt;
}
</style>
