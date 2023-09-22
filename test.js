const { getXirr } = require('./index.js')

const today = new Date()
const years = Array.from({ length: 6 }).map((_, idx) => today.getFullYear() - idx)

console.log(
  getXirr(
    [-1000, 100, 100, 100, 100, 1000],
    years.map((n) => new Date(n).toISOString()),
  ),
)
