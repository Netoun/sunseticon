<template>
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
        <img
          src="@/assets/sunset.png"
          alt="avatar"
        >
      </v-avatar>
      <span class="hidden-sm-and-down">Sunseticon</span>
    </v-toolbar-title>
    <v-spacer />
    <v-text-field
      flat
      solo-inverted
      hide-details
      prepend-inner-icon="search"
      label="Search"
    ></v-text-field>
    <v-spacer></v-spacer>

  </v-toolbar>
</template>

<style scoped>
.avatar {
  margin-right: 10px;
}
</style>

