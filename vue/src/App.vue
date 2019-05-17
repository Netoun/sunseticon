<template>
  <v-app class="sunseticon">
    <router-view></router-view>
  </v-app>
</template>

<script>
export default {
  name: 'App',
  data () {
    return {
      //
    }
  }
}
</script>

<style scoped>
.sunseticon {
  background-color: white !important;
}
</style>
