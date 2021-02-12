<template>
  <a class="link" :href="href">{{ text }}</a>
</template>

<script>
export default {
  name: 'Link',
  props: {
    text: String,
    href: String
  }
}
</script>

<style scoped>
.link {
  text-decoration: none;
}

.link:hover {
  text-decoration: underline;
}
</style>
