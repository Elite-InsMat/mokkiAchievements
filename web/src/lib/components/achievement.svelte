<script lang="ts">
	import { DateTime } from 'luxon';
	export let name: string;
	export let description: string;
	export let image: string;
	export let id: number;
	export let unlocked: boolean;
	export let timestamp: number;

	const handleCheck = () => {
		timestamp = Date.now();
		unlocked = true;
		fetch('https://mokki-achievements.shuttleapp.rs/user/unlock_achievement', {
			method: 'PUT',
			headers: {
				authorization: `Bearer ${window.localStorage.getItem('token')}`
			},
			body: JSON.stringify({
				id,
				timestamp
			})
		});
	};
</script>

<div class="achievement">
	<div class={`image ${unlocked ? 'image-unlocked' : 'image-locked'}`}>
		<img src={'data:image/svg+xml;base64,' + image} alt="" />
	</div>
	<div class="text">
		<h2>{name}</h2>
		<p>{description}</p>
		{#if unlocked}
			<p>{DateTime.fromMillis(timestamp).toFormat('yyyy-MM-dd:HH.mm.ss')}</p>
		{/if}
	</div>
	<input
		class="checkbox"
		type="checkbox"
		disabled={unlocked}
		bind:checked={unlocked}
		on:click={handleCheck}
	/>
</div>

<style lang="scss">
	.achievement {
		display: flex;
		border: solid black;
		margin-bottom: 1vh;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		justify-items: center;
		text-align: center;
	}
	.checkbox {
		min-height: 64px;
		min-width: 64px;
		margin: 10px;
		justify-self: flex-end;
		cursor: pointer;
	}
	.image {
		img {
			width: 96px;
			margin: 10px;
		}
	}
	.image-locked {
		filter: saturate(0);
	}
</style>
