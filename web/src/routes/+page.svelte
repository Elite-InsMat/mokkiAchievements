<script lang="ts">
	import { onMount } from 'svelte';
	import Achievement from '$lib/components/achievement.svelte';
	import { goto } from '$app/navigation';
	import type { User } from '$lib/types';
	import { userNameStore } from '$lib/stores/user';
	import { BASE_URL } from '$lib/config';

	let achievements: Achievement[] = [];
	let user: User = {} as User;
	onMount(async () => {
		if (!window.localStorage.getItem('token')) {
			goto('/login');
		} else {
			try {
				const userResponse = await fetch(`${BASE_URL}/user/get_user`, {
					headers: {
						authorization: `Bearer ${window.localStorage.getItem('token')}`
					}
				});
				user = await userResponse.json();
				$userNameStore = user.username;
				const achievementsResponse = await fetch(`${BASE_URL}/achievement/get_achievements`);
				achievements = await achievementsResponse.json();
			} catch (error) {
				goto('/login');
			}
		}
	});
</script>

<div class="app">
	<div class="header">
		<h1>Mökki achievements</h1>
		<p>Logged in as: {$userNameStore}</p>
	</div>
	{#each achievements as achievement, index}
		<Achievement
			name={achievement.name}
			description={achievement.description}
			image={achievement.image}
			id={achievement.id}
			unlocked={user?.achievements.includes(achievement.id) || false}
			timestamp={user.timestamps[index]}
		/>
	{/each}
</div>

<style lang="scss">
	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
</style>
