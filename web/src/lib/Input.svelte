<script>
  let text = "";
  async function submit() {
    const response = await fetch("/create_post", {
      method: "POST", // *GET, POST, PUT, DELETE, etc.
      mode: "cors", // no-cors, *cors, same-origin
      cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
      credentials: "same-origin", // include, *same-origin, omit
      headers: {
        "Content-Type": "plain/text",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      redirect: "follow", // manual, *follow, error
      referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
      body: text, // body data type must match "Content-Type" header
    });
  }
</script>

<textarea class="textarea" bind:value={text} placeholder="时间，地点，事件" />

<button on:click={submit}> 提交 </button>

<style>
  .textarea {
    width: 400px;
    height: 200px;
  }
</style>
