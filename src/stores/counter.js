import { defineStore } from 'pinia'


export const useCounterStore = defineStore('counter', {
  state: () => ({
    count: 10,
    valueKw: 50,
    valueKw1: 10,
    valueKw2: 30,
  }),
  actions: {
    increment() {
      this.count++
    },
    decrement() {
      this.count--
    },
    incKw() {
      this.valueKw += 5
    },
    decKw() {
      this.valueKw -= 5
    }
  }
})