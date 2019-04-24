<template>
  <v-app id="inspire">
    <v-toolbar
      :clipped-left="$vuetify.breakpoint.lgAndUp"
      color="primary"
      dark
      app
      fixed
    >
      <v-toolbar-title>
        <v-avatar
          color="primary"
          class="avatar"
        > 
          <img src="@/assets/sunset.png" alt="avatar">
        </v-avatar>
        <span class="hidden-sm-and-down">{{ title }}</span>
      </v-toolbar-title>
      <v-spacer/>
      <v-text-field
        flat
        solo-inverted
        hide-details
        prepend-inner-icon="search"
        label="Search"     
      ></v-text-field>
      <v-spacer></v-spacer>

    </v-toolbar>
    <v-content>
      <v-container fluid fill-height>s
        <v-layout justify-center align-center>
          
        </v-layout>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
  export default {
    data: () => ({
      title: ''
    }),
    mounted() {
      fetch('http://localhost:8000/hello')
      .then(response =>{
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
