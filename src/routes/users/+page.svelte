<script lang="ts">
	import { onMount } from 'svelte';

	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
	import { open } from '@tauri-apps/plugin-dialog';

	import { notify } from '$lib/components/Notification.svelte';
	import Grid from '$lib/components/Grid.svelte';

	import { command, getAvatarUrl, Platform } from '$lib';

	let users = $state([]) as User[];
	let filteredUsers = $state([]) as User[];
	let loading = $state(false);

	let filter = $state(Platform.Twitch);
	let channelName = $state('');
	let search = $state('');

	async function addUser(username: string) {
		loading = true;

		if (!username) {
			notify('No username provided');
			return;
		}

		await command('add_user', { username, platform: filter }).then(() => {
			notify(`Added '${username}'`);
		});

		loading = false;
	}

	async function updateUser(username: string, platform: Platform) {
		loading = true;
		await command('add_user', { username, platform }).then(() => {
			notify(`Updated '${username}'`);
		});

		loading = false;
	}

	async function removeUser(username: string, platform: Platform) {
		await command('remove_user', { username, platform }).then(() => {
			notify(`Removed '${username}'`);
		});
	}

	function handleSearch(event: Event) {
		const target = event.target as HTMLInputElement;

		if (target.value) {
			filteredUsers = users.filter((user) =>
				user.username.toLowerCase().includes(target.value.toLowerCase())
			);
		} else {
			filteredUsers = users;
		}
	}

	async function importSubscriptions() {
		const subscriptionsFilePath = await open({
			multiple: false,
			directory: false,
			filters: [{ name: 'CSV', extensions: ['csv'] }]
		});

		if (!subscriptionsFilePath) {
			notify('No file selected');
			return;
		}

		await command<number>('import_subscriptions', { subscriptionsFilePath }).then(async (data) => {
			if (data === 0) {
				notify('No new subscriptions retrieved');
				return;
			}

			if (!data) return;

			notify(`Imported ${data} subscriptions`);
			await updateView();
		});
	}

	async function updateView() {
		await command<User[]>('get_users').then((data) => {
			if (!data) return;
			users = data.sort((a, b) => a.username.localeCompare(b.username));
			filteredUsers = users.filter((user) =>
				user.username.toLowerCase().includes(search.toLowerCase())
			);
		});
	}

	onMount(async () => {
		const appWebview = getCurrentWebviewWindow();
		appWebview.listen<string>('updated_users', async () => {
			await updateView();
		});

		loading = true;
		await updateView();
		loading = false;
	});
</script>

<div class="flex h-full w-full flex-col gap-3 p-2">
	<div class="mx-4 flex items-center gap-2">
		<select
			bind:value={filter}
			class="rounded-md border border-gray-600 bg-gray-800 px-2 py-1 focus:ring-2 focus:ring-blue-500 focus:outline-none"
		>
			<option value={Platform.Twitch} class="bg-gray-800">Twitch</option>
			<option value={Platform.YouTube} class="bg-gray-800">YouTube</option>
		</select>

		<hr class="mx-1 h-full border-gray-700" />

		<form onsubmit={async () => await addUser(channelName)}>
			<input
				type="text"
				bind:value={channelName}
				placeholder="Add by channel name"
				class="rounded-md border border-gray-600 bg-gray-800 px-3 py-1 focus:ring-2 focus:ring-blue-500 focus:outline-none"
			/>
		</form>

		<input
			type="text"
			bind:value={search}
			placeholder="Search"
			oninput={handleSearch}
			class="rounded-md border border-gray-600 bg-gray-800 px-3 py-1 focus:ring-2 focus:ring-blue-500 focus:outline-none"
		/>

		{#if filter === Platform.YouTube}
			<button
				onclick={async () => await importSubscriptions()}
				class="cursor-pointer rounded-md border border-gray-600 bg-gray-800 px-4 py-1 hover:ring-2 hover:ring-blue-500 hover:outline-none"
			>
				Import subscriptions
			</button>
		{/if}
	</div>

	<hr class="w-full border-gray-700" />

	<div class="flex w-full">
		{#if !loading && filteredUsers.filter((user) => user.platform === filter).length === 0}
			<span class="text-lg font-medium">No users found</span>
		{:else}
			<Grid>
				{#each filteredUsers as user, index (index)}
					{#if user.platform === filter}
						<div class="flex flex-col items-center">
							<img
								src={getAvatarUrl(user.platform, user.username, user.avatar)}
								id={user.username}
								alt={'Avatar of ' + user.username}
								class="h-16 w-16 rounded-full"
							/>

							<div class="flex w-full flex-col items-center justify-between">
								<span title={user.id} class="text-lg font-medium">{user.username}</span>

								<div class="flex w-full">
									<button
										disabled={loading}
										title={filter === Platform.Twitch
											? 'Update emotes and avatar'
											: 'Update avatar'}
										class="block w-full cursor-pointer bg-neutral-500 px-2 py-1 hover:bg-neutral-600"
										onclick={async () => await updateUser(user.username, user.platform)}
									>
										Update
									</button>

									<button
										disabled={loading}
										title="Remove user"
										class="block cursor-pointer bg-red-500 px-2 py-1 hover:bg-red-600"
										onclick={async () => await removeUser(user.username, user.platform)}
									>
										<svg
											xmlns="http://www.w3.org/2000/svg"
											width="1em"
											height="1em"
											viewBox="0 0 2048 2048"
											><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
												fill="currentColor"
												d="M1792 384h-128v1472q0 40-15 75t-41 61t-61 41t-75 15H448q-40 0-75-15t-61-41t-41-61t-15-75V384H128V256h512V128q0-27 10-50t27-40t41-28t50-10h384q27 0 50 10t40 27t28 41t10 50v128h512zM768 256h384V128H768zm768 128H384v1472q0 26 19 45t45 19h1024q26 0 45-19t19-45zM768 1664H640V640h128zm256 0H896V640h128zm256 0h-128V640h128z"
											/></svg
										>
									</button>
								</div>
							</div>
						</div>
					{/if}
				{/each}
			</Grid>
		{/if}
	</div>
</div>
