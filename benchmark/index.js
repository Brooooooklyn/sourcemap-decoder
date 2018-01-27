const { SourceMapConsumer } = require('source-map')
const microtime = require('microtime')
const { join } = require('path')

const sourcemap = require('./map.json')

const { SourceMapDecoder } = require('../index')

const mapPath = join(process.cwd(), 'benchmark', 'map.json')

const sourcemapDecoder = new SourceMapDecoder(mapPath)

;(async () => {
  const sourcemapConsumer = await new SourceMapConsumer(sourcemap)
  const startJavascript = microtime.now()
  const jsResult = sourcemapConsumer.originalPositionFor({
    line: 1,
    column: 195778
  })
  const javascriptEndTime = microtime.now()
  const startRust = microtime.now()
  const rustResult = sourcemapDecoder.parse(1, 195778)
  const rustEndTime = microtime.now()
  console.log('JavaScript parse time', javascriptEndTime - startJavascript, 'microseconds')
  console.log('Rust parse time:', rustEndTime - startRust, 'microseconds')
  console.log(`JavaScript parse result, Source: ${jsResult.source}, Line: ${jsResult.line}`)
  console.log(`Rust parse result, Source: ${rustResult[0]}, Line: ${rustResult[1]}`)
})()
