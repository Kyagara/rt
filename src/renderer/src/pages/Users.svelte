<script lang="ts">
	import { onMount } from 'svelte'

	import { notify } from '$lib/components/Notification.svelte'
	import Grid from '$lib/components/Grid.svelte'

	import { getAvatarUrl } from '$lib'
	import { Platform } from '$shared/enums'
	import { fade } from 'svelte/transition'

	let users = $state([]) as User[]
	let filteredUsers = $state([]) as User[]
	let loading = $state(true)

	let filter = $state(Platform.Twitch)
	let channelName = $state('')
	let search = $state('')

	async function addUser() {
		if (!channelName) {
			notify('No username provided')
			return
		}

		if (users.find((user) => user.username === channelName)) {
			notify(`'${channelName}' is already added`)
			return
		}

		loading = true

		try {
			const user = await window.user.add(filter, channelName)
			notify(`Added '${user.username}'`)
			users.push(user)
			users = users.sort((a, b) => a.username.localeCompare(b.username))
		} catch (err) {
			notify(`Could not add '${channelName}'`, err)
		}

		loading = false
	}

	async function updateUser(username: string) {
		loading = true

		try {
			const user = await window.user.add(filter, username)
			notify(`Updated '${user.username}'`)
			users = users.map((user) => (user.username === username ? user : user))
		} catch (err) {
			notify(`Could not update '${username}'`, err)
		}

		loading = false
	}

	async function removeUser(username: string) {
		loading = true

		try {
			const deletedUsername = await window.user.remove(filter, username)
			notify(`Removed '${deletedUsername}'`)
			users = users.filter((user) => user.username !== deletedUsername)
		} catch {
			notify(`Could not remove '${username}'`)
		}

		loading = false
	}

	$effect(() => {
		filteredUsers = users.filter(
			(user) =>
				user.username.toLowerCase().includes(search.toLowerCase()) && user.platform === filter
		)
	})

	async function importSubscriptions() {
		loading = true

		try {
			const imported = await window.video.import()

			if (imported === -1) return

			if (imported === 0) {
				notify('No new subscriptions retrieved')
				return
			}

			users = await window.user.list()

			notify(`Imported ${imported} subscriptions`)
		} catch (err) {
			notify('Error importing subscriptions', err)
		}

		loading = false
	}

	onMount(async () => {
		try {
			const data = await window.user.list()
			users = data.sort((a, b) => a.username.localeCompare(b.username))
		} catch {
			notify('Error getting users')
		}

		loading = false
	})
</script>

<div data-simplebar data-simplebar-auto-hide="false" class="flex h-full w-full">
	<div class="flex h-full w-full flex-col gap-2 p-1">
		<div class="flex items-center gap-2">
			<select
				bind:value={filter}
				disabled={loading}
				class="border border-gray-600 bg-neutral-900 px-2 py-1 {loading
					? ''
					: 'cursor-pointer focus:ring-2 focus:ring-blue-500 focus:outline-none'}"
			>
				<option value={Platform.Twitch} class="bg-neutral-900">Twitch</option>
				<option value={Platform.YouTube} class="bg-neutral-900">YouTube</option>
			</select>

			<form
				onsubmit={async (e) => {
					e.preventDefault()
					await addUser()
				}}
			>
				<input
					type="text"
					bind:value={channelName}
					autocomplete="off"
					spellcheck="false"
					placeholder="Channel name"
					disabled={loading}
					class="border border-gray-600 bg-neutral-900 px-3 py-1 {loading
						? ''
						: 'focus:ring-2 focus:ring-blue-500 focus:outline-none'}"
				/>
			</form>

			<input
				type="text"
				bind:value={search}
				placeholder="Search"
				autocomplete="off"
				spellcheck="false"
				disabled={loading}
				class="border border-gray-600 bg-neutral-900 px-3 py-1 {loading
					? ''
					: 'focus:ring-2 focus:ring-blue-500 focus:outline-none'}"
			/>

			{#if filter === Platform.YouTube}
				<button
					onclick={async () => await importSubscriptions()}
					transition:fade={{ duration: 50 }}
					disabled={loading}
					class="border border-gray-600 bg-neutral-900 px-4 py-1 {loading
						? ''
						: 'cursor-pointer hover:bg-neutral-700'}"
				>
					Import subscriptions
				</button>
			{/if}
		</div>

		<hr class="w-full border-gray-700 pb-2" />

		<div class="flex w-full">
			{#if !loading && filteredUsers.filter((user) => user.platform === filter).length === 0}
				<span class="flex w-full justify-center text-lg font-medium">No users found</span>
			{:else}
				<Grid>
					{#each filteredUsers as user, index (index)}
						<div class="flex flex-col items-center">
							<img
								src={getAvatarUrl(user.platform, user.username, user.avatar)}
								id={user.username}
								alt={'Avatar of ' + user.username}
								class="h-16 w-16 rounded-full"
							/>

							<div class="flex w-full flex-col items-center justify-between">
								<span title={user.username} class="text-lg font-medium">{user.display_name}</span>

								<div class="flex w-full justify-center">
									<button
										disabled={loading}
										title={filter === Platform.Twitch
											? 'Update emotes and avatar'
											: 'Update avatar'}
										class="w-full max-w-1/2 bg-neutral-700 px-2 py-1 {loading
											? ''
											: 'cursor-pointer hover:bg-neutral-600'}"
										onclick={async () => await updateUser(user.username)}
									>
										Update
									</button>

									<button
										disabled={loading}
										title="Remove user"
										class="bg-red-500 px-2 py-1 {loading ? '' : 'cursor-pointer hover:bg-red-600'}"
										onclick={async () => await removeUser(user.username)}
									>
										<svg
											xmlns="http://www.w3.org/2000/svg"
											width="1rem"
											height="1rem"
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
					{/each}
				</Grid>
			{/if}
		</div>
	</div>
</div>
