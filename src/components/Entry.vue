<template>
  <div class="entry">
    <div class="entry-header">
      <!-- TODO: add link -->
      <h4 class="title">
        <Link v-if="entry.link" :text="entry.title" :href="entry.link" />
        <span v-else>{{ entry.title }}</span>
      </h4>

      <h5 class="subtitle" v-for="subtitle in entry.subtitles" :key="subtitle.key">{{ subtitle.text }}</h5>
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
    // title: String,
    // subtitles: Array,
    // description: String,
    // link: String
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

.entry p, li {
  font-size: 11pt;
  color: rgb(30, 30, 30);
}
</style>
