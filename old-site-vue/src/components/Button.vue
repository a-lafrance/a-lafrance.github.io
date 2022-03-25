<template>
  <div class="button">
    <Link class="navy" :text="title" :href="destination" />
  </div>
</template>

<script>
import Link from './Link.vue'

export default {
  name: 'Button',
  components: { Link },
  props: {
    title: String,
    destination: String
  }
}
</script>

<style scoped>
.button {
  border-radius: 10px;

  background-color: rgb(230, 230, 230);
  text-align: center;

  padding-top: 15px;
  padding-bottom: 15px;
  padding-left: 25px;
  padding-right: 25px;

  margin-right: 2vw;
  margin-bottom: 2vh;

  font-size: 14pt;
}
</style>
