<template>
  <div class="section" :id="name">
    <slot></slot>
  </div>

  <hr />
</template>

<script>
export default {
  name: 'BlankSection',
  props: {
    name: String
  }
}
</script>

<style scoped>
.section {
  width: 84vw;

  margin-left: 6vw;
  margin-right: 6vw;
  margin-top: 2vh;
  margin-bottom: 2vh;

  padding-top: 2vh;
  padding-left: 1.5vw;
  padding-right: 1.5vw;
  padding-bottom: 2vh;

  border-radius: 20px;

  font-family: "Helvetica", sans-serif;
  font-weight: 100;
  line-height: 140%;
}

hr {
  margin-left: 3vw;
  margin-right: 3vw;

  background-color: rgb(0, 60, 120);
  border-color: rgb(0, 60, 120);
}
</style>
