<script lang="ts">
	import { goto } from '$app/navigation';
	import Credentials from '$lib/components/credentials.svelte';
	import { BASE_URL } from '$lib/config';
	import { userNameStore } from '$lib/stores/user';
	const handleLogin = async (e: any, username: string, password: string) => {
		e.preventDefault();
		const data = await fetch(`${BASE_URL}/user/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username,
				password
			})
		});
		let jsonData = await data.json();
		window.localStorage.setItem('token', jsonData.token);
		$userNameStore = jsonData.username;
		goto('/');
	};
</script>

<div>
	<h2>Login</h2>
	<h3 style="cursor:pointer" on:click={() => goto('/register')}>No account?</h3>
	<Credentials handleSubmit={handleLogin} />
</div>
