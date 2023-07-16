html
<script type='text/javascript'>
 fetch('hello.wasm')
  .then(response => response.arrayBuffer())
  .then(buffer => WebAssembly.compile(buffer))
  .then(console.log)
</script>
