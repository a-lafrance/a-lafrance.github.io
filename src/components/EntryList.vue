<template>
  <Section :name="name" :title="title">
    <Entry v-for="(entry, index) in entries" :key="index" :entry="entry" />
  </Section>
</template>

<script>
import Section from './Section.vue'
import Entry from './Entry.vue'

export default {
  name: 'EntryList',
  components: { Section, Entry },
  props: {
    name: String,
    title: String,
    entries: Array
  }
}
</script>

<style scoped>
.section-content {
  display: flex;
  flex-flow: column;
}

.section-title {
  color: rgb(0, 60, 120);
}
</style>
