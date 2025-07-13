<script lang="ts">
const html = document.querySelector('html')
html.setAttribute("data-bs-theme", "dark")

import '../node_modules/bootstrap/dist/css/bootstrap.css'
import '../node_modules/bootstrap/dist/js/bootstrap.min.js'
import '../node_modules/@popperjs/core/dist/umd/popper.min.js'
import { invoke } from "@tauri-apps/api/core";
import { attachConsole } from "@tauri-apps/plugin-log";

export default {
  data()  {
    return {
      useExtended: false,
      useTime: true,
      seed: '',
      outputLength: 12,
      output: '',
      previewText: ''
    }
  },
  methods: {
    async generateOutput() {
      this.output = await invoke('generate_output', { useExtended: this.useExtended, useTime: this.useTime, seed: this.seed, length: this.outputLength })
      this.previewText = '';
      for (let i = 0; i < Math.min(this.output.length, 100); i++) {
        this.previewText += this.output[i];
      }
    },
    handleSeedInput(event) {
      if (event.target.value.length < 19) {
        this.seed = event.target.value
      }
    },
    handleCheckBox(event) {
      if (event.target.id == 'system-time') {
        this.useTime = !this.useTime
        event.target.checked = this.useTime
      } else if (event.target.id == 'extended-ascii') {
        this.useExtended = !this.useExtended
        event.target.checked = this.useExtended
      }
    },
    handleNumberInput(event) {
      if (event.target.value < 8) {
        event.target.value = 8
      } else if (event.target.value > 1000) {
        event.target.value = 1000
      }

      this.outputLength = parseInt(event.target.value)
    },
    async copyToClipboard() {
      await navigator.clipboard.writeText(this.output)
    },
    async setup() {
      await invoke('setup')
    }
  },
  beforeMount() {
    attachConsole()
    this.setup()
  }
}
</script>

<template>
<div class="container mt-3">
  <h3>Entropy Generator</h3>
  <form>
    <div class="form-group row">
      <label for="seed" class="col-auto col-form-label">Custom Seed (Max 19 Characters):</label>
      <div class="col-auto">
        <input type="text" class="form-text form-input" id="seed" @input.prevent="handleSeedInput" :value="seed" maxlength="19">
      </div>
    </div>
    <div class="form-group row">
      <label for="useTime" class="col-auto col-form-label">Use System Time:</label>
      <div class="col-auto">
        <input type="checkbox" id="system-time" class="form-input" :checked="useTime" @click="handleCheckBox">
      </div>
    </div>
    <div class="form-group row">
      <label for="extraAscii" class="col-auto col-form-label">Include Extended ASCII (128-255):</label>
      <div class="col-auto">
        <input type="checkbox" class="form-input" id="extended-ascii" :checked="useExtended" @click="handleCheckBox">
      </div>
    </div>
    <div class="form-group row">
      <label for="outLength" class="col-auto col-form-label">Output Length (8-1000):</label>
      <div class="col-auto">
        <input type="number" placeholder="12" class="form-input" :value="outputLength" @input.prevent="handleNumberInput">
      </div>
    </div>
    <div class="form-group">
      <label for="outText">Live Preview (first 100 character):</label>
      <textarea class="form-control" readonly maxlength="100">{{previewText}}</textarea>
    </div>
    <div id="breh">
      <button class="btn btn-warning" @click.prevent="generateOutput()">Generate</button>
      <button class="btn btn-primary" @click.prevent="copyToClipboard()">Copy</button>
    </div>
  </form>
</div>
</template>

<style>
@font-face {
  font-family: dos-vga;
  src: url("./assets/fonts/dos-vga-9x16.ttf");
}

body {
  padding: 20px;
  font-family: dos-vga,serif;
  font-size: 0.75em;
}

form {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

form .form-group {
  margin-left: 5px;
  margin-bottom: 10px;
}

form .form-input {
  position: absolute;
  left: 50%;
}

form #breh button {
  margin-right: 10px;
}

textarea {
  resize: none;
  height: 16ch;
}
</style>