<template>
  <v-container
    grid-list-md
    text-xs-center
    fluid
  >
    <v-layout
      row
      wrap
    >
      <v-flex
        v-for="icon in icons"
        :key="`1${icon.name}`"
        xs1
        class="icon"
      >
        <v-card
          class="card-icon"
          flat
          @click="download(icon)"
        >
          <v-img
            :src="icon.url"
            contain
            class="icon"
          />
        </v-card>
      </v-flex>
    </v-layout>
  </v-container>
</template>

<script>
// import Icon from './Icon'

export default {
  data: () => ({
    dialog: false,
    icons: [{
      name: 'client',
      url: 'http://localhost:8000/icon/icon_client.png',
    }, {
      name: 'server',
      url: 'http://localhost:8000/icon/icon_server.png',
    }],
    icon: {}
  }),
  components: {
  },
  methods: {
    download (icon) {
      fetch(icon.url).then(response => response.blob()).then(image => {
        const url = window.URL.createObjectURL(new Blob([image]));
        const link = document.createElement('a');
        link.href = url;
        link.setAttribute('download', `${icon.name}.png`)
        document.body.appendChild(link);
        link.click();
      })
    }
  }
}
</script>

<style scoped>
.avatar {
  margin-right: 10px;
}

.icon {
  padding: 10px;
}

.card-icon {
  border-radius: 5px;
  transition-duration: 0.5s;
  padding: 10px;
}

.card-icon:hover {
  transform: scale(1.1);
  background: rgb(214, 214, 214);
}
</style>
