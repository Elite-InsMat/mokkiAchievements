<script lang="ts">
	export let handleSubmit: any;
	let username: string;
	let password: string;
</script>

<form on:submit={(e) => handleSubmit(e, username, password)}>
	<label>Username: </label>
	<input bind:value={username} type="text" />
	<label>Password: </label>
	<input bind:value={password} type="password" />
	<button type="submit">Login</button>
</form>
