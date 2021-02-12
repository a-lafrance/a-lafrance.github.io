<template>
  <div class="entry">
    <div class="entry-header">
      <h4 class="title navy">
        <Link class="navy" v-if="entry.link" :text="entry.title" :href="entry.link" />
        <span v-else>{{ entry.title }}</span>
      </h4>

      <h5 class="subtitle" v-for="(subtitle, index) in entry.subtitles" :key="index">{{ subtitle }}</h5>
    </div>

    <p class="description">{{ entry.description }}</p>
  </div>
</template>

<script>
import Link from './Link.vue'

export default {
  name: 'Entry',
  components: { Link },
  props: {
    entry: Object
  },
}
</script>

<style scoped>
.title {
  color: rgb(0, 60, 120);
  font-size: 13pt;

  margin-top: 2vh;
  margin-bottom: 0;
}

.subtitle {
  margin-top: 0.75vh;
  margin-bottom: 0;

  font-size: 11pt;
  color: rgb(30, 30, 30);
}

.description {
  font-size: 11pt;
  color: rgb(30, 30, 30);
}
</style>
