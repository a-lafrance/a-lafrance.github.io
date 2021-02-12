<template>
  <BlankSection :name="name">
    <h2 class="section-title">{{ title }}</h2>

    <div class="section-content">
      <slot></slot>
    </div>
  </BlankSection>
</template>

<script>
import BlankSection from './BlankSection.vue'

export default {
  name: 'Section',
  components: { BlankSection },
  props: {
    name: String,
    title: String,
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
  font-size: 20pt;
  font-weight: normal;
}
</style>
