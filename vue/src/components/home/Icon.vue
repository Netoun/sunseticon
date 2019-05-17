<template>
  <v-dialog
    :value="value"
    @input="$emit('input')"
    max-width="500"
  >
    <v-card>
      <v-card-title class="headline">{{icon.name}}</v-card-title>
      <v-layout
        row
        wrap
      >
        <v-img :src="require(`@/assets/${icon.url}`)" />
        <v-card-actions>
          <v-spacer></v-spacer>

          <v-btn
            color="green darken-1"
            flat="flat"
            @click="dialog = false"
          >
            Download
          </v-btn>
        </v-card-actions>
      </v-layout>
    </v-card>
  </v-dialog>
</template>

<script>
export default {
  props: ['value', 'icon'],
  data: () => ({
  })
}
</script>

<style scoped>
</style>

