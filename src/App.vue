<template>
  <List :items="this.items" />
</template>

<script>
import List from './components/List.vue'

export default {
  name: 'App',
  components: {
    List
  },
  data: function() {
    return {
      items: [
        { id: 1, text: 'hello, world' },
        { id: 2, text: 'asg' },
        { id: 3, text: 'acya' },
      ]
    }
  }
}
</script>

<style>

</style>
