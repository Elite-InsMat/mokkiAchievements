<script lang="ts">
	import { goto } from '$app/navigation';
	import { BASE_URL } from '$lib/config';
	import { userNameStore } from '$lib/stores/user';
	import Credentials from '$lib/components/credentials.svelte';
	const handleRegister = async (e: any, username: string, password: string) => {
		e.preventDefault();
		try {
			const data = await fetch(`${BASE_URL}/user/register`, {
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
		} catch (error) {
			goto('/login');
		}
	};
</script>

<div>
	<h2>Register</h2>
	<Credentials handleSubmit={handleRegister} />
</div>
