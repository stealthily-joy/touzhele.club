<script>
  import Input from "./lib/Input.svelte";

  async function list_post() {
    const res = await fetch(`/list_post`);
    const text = await res.text();
    return text;
  }

  let promise = list_post();
</script>

<main>
  <Input />
  <div>
    {#await promise}
      <p>loading...</p>
    {:then text}
      <div class="text">{text}</div>
    {:catch error}
      <p style="color: red">{error.message}</p>
    {/await}
  </div>
</main>

<style>
  .text {
    white-space: pre-wrap;
  }
</style>
