<template>
  <v-app id="inspire">
    <v-content>
      <v-container
        fluid
        fill-height
      >
        <v-layout
          justify-center
          align-center
        >

        </v-layout>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
export default {
  data: () => ({
    title: '',
    list: []
  }),
  mounted () {
    fetch('http://localhost:8000/hello')
      .then(response => {
        return response.json()
      })
      .then(data => {
        this.title = data.title
      })
  },
}
</script>

<style scoped>
.avatar {
  margin-right: 10px;
}
</style>
