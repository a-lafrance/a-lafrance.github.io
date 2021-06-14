<template>
  <ul class="list">
    <div v-for="(item, i) in items" :key="i">
      <li class="list-item" :key="i">
        {{ item.text }}
      </li>

      <li class="list-item list-subitem" v-for="(subitem, j) in item.subitems" :key="j">
        {{ subitem }}
      </li>
    </div>
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
.list {
  padding-left: 2vh;
}

.list-item {
  font-size: 11pt;
}

.list-subitem {
  margin-left: 2vw;
}
</style>
