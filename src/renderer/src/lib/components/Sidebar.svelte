<script lang="ts">
	import { currentView, changeView } from '$lib/state/View.svelte'
	import { View } from '$shared/enums'

	function handleMouseWheelClick(event: MouseEvent, view: View) {
		if (event.button === 1) {
			openNewWindow(view)
		}
	}

	function openNewWindow(view: View) {
		localStorage.setItem('lastView', view)
		window.main.newWindow(`/${view}`)
	}
</script>

<aside class="user-select-none flex w-12 min-w-12 flex-col items-center gap-2 bg-neutral-800">
	<div class="flex w-full flex-col items-center">
		<button
			aria-label="Videos"
			title="Videos"
			onmousedown={(event: MouseEvent) => handleMouseWheelClick(event, View.Videos)}
			onclick={() => changeView(View.Videos)}
			disabled={currentView.route === '/videos'}
			class="flex w-full flex-col items-center py-2 {currentView.route === '/videos'
				? 'opacity-50 duration-100 ease-in-out'
				: 'cursor-pointer hover:bg-neutral-600'}"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="m2048 1544l-512-256v248H0V512h1536v248l512-256zm-640-904H128v768h1280zm512 71l-384 193v240l384 193z"
				/></svg
			>
		</button>

		<button
			aria-label="Streams"
			title="Streams"
			onmousedown={(event: MouseEvent) => handleMouseWheelClick(event, View.Streams)}
			onclick={() => changeView(View.Streams)}
			disabled={currentView.route === '/streams'}
			class="flex w-full flex-col items-center py-2 {currentView.route === '/streams'
				? 'opacity-50 duration-100 ease-in-out'
				: 'cursor-pointer hover:bg-neutral-600'}"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="M1024 779q51 0 95 19t78 53t52 77t20 96q0 51-19 95t-53 78t-77 52t-96 20q-51 0-95-19t-78-53t-52-77t-20-96q0-51 19-95t53-78t77-52t96-20m0 384q29 0 54-11t44-29t30-44t11-55t-11-54t-29-44t-44-30t-55-11t-54 11t-44 29t-30 44t-11 55t11 54t29 44t44 30t55 11m716-855q72 71 127 154t93 174t57 189t20 199q0 101-19 199t-58 189t-93 174t-127 154l-75-75q64-64 113-138t83-156t51-169t18-178q0-90-17-177t-51-170t-83-156t-114-138zM383 383q-64 64-113 138t-84 156t-51 169t-18 178q0 90 17 177t52 170t83 156t114 138l-75 75q-72-71-127-154t-93-174t-57-189t-20-199q0-101 19-199t58-189t93-174t127-154zm1086 196q89 90 136 204t48 241q0 126-47 240t-137 205l-75-75q74-74 113-169t40-201q0-105-39-200t-114-170zm-815 75q-74 74-113 169t-40 201q0 105 39 200t114 170l-75 75q-89-90-136-204t-48-241q0-126 47-240t137-205z"
				/></svg
			>
		</button>

		<button
			aria-label="Users"
			title="Users"
			onmousedown={(event: MouseEvent) => handleMouseWheelClick(event, View.Users)}
			onclick={() => changeView(View.Users)}
			disabled={currentView.id === View.Users}
			class="flex w-full flex-col items-center py-2 {currentView.id === View.Users
				? 'opacity-50 duration-100 ease-in-out'
				: 'cursor-pointer hover:bg-neutral-600'}"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="M1397 1550q-21-114-78-210t-141-166t-189-110t-221-40q-88 0-170 23t-153 64t-129 100t-100 130t-65 153t-23 170H0q0-117 35-229t101-207t157-169t203-113q-56-36-100-83t-76-103t-47-119t-17-129q0-106 40-199t109-163T568 40T768 0t199 40t163 109t110 163t40 200q0 66-16 129t-48 119t-75 103t-101 83q99 38 183 100t147 143t105 177t54 202l-57 58zM384 512q0 80 30 149t82 122t122 83t150 30q79 0 149-30t122-82t83-122t30-150q0-79-30-149t-82-122t-123-83t-149-30q-80 0-149 30t-122 82t-83 123t-30 149m1645 941l-557 558l-269-270l90-90l179 178l467-466z"
				/></svg
			>
		</button>

		<button
			aria-label="Settings"
			title="Settings"
			onmousedown={(event: MouseEvent) => handleMouseWheelClick(event, View.Settings)}
			onclick={() => changeView(View.Settings)}
			disabled={currentView.id === View.Settings}
			class="flex w-full flex-col items-center py-2 {currentView.id === View.Settings
				? 'opacity-50 duration-100 ease-in-out'
				: 'cursor-pointer hover:bg-neutral-600'}"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="M1783 988v18q0 9 1 18v18q0 9-1 18l259 161l-159 383l-297-68q-24 26-50 50l68 297l-383 159l-161-259h-18q-9 0-18 1h-18q-9 0-18-1l-161 259l-383-159l68-297q-26-24-50-50l-297 68L6 1221l259-161v-18q0-9-1-18v-18q0-9 1-18L6 827l159-383l297 68q24-26 50-50l-68-297L827 6l161 259h18q9 0 18-1h18q9 0 18 1L1221 6l383 159l-68 297q26 24 50 50l297-68l159 383zm-117 130q2-24 4-47t2-48q0-23-2-47t-4-47l236-147l-86-208l-271 63q-31-38-63-70t-71-64l63-271l-208-86l-148 236q-23-2-47-4t-47-2q-24 0-47 2t-48 4L782 146l-208 86l63 271q-38 31-70 63t-64 71l-271-63l-86 208l236 148q-2 24-4 47t-2 48q0 23 2 47t4 47l-236 147l86 208l271-63q31 38 63 70t71 64l-63 271l208 86l148-236q23 2 47 4t47 2q24 0 47-2t48-4l147 236l208-86l-63-271q38-31 70-63t64-71l271 63l86-208zm-642-470q78 0 146 29t120 81t80 119t30 147q0 78-29 146t-81 120t-119 80t-147 30q-78 0-146-29t-120-81t-80-119t-30-147q0-78 29-146t81-120t119-80t147-30m0 640q55 0 103-20t84-57t56-84t21-103q0-55-20-103t-57-84t-84-56t-103-21q-55 0-103 20t-84 57t-56 84t-21 103q0 55 20 103t57 84t84 56t103 21"
				/></svg
			>
		</button>
	</div>

	<hr class="w-full border-gray-600" />

	<div class="flex h-full w-full flex-col items-center">
		<div class="flex-1"></div>

		<button
			aria-label="Open new window"
			title="Open new window"
			onmousedown={(event: MouseEvent) => handleMouseWheelClick(event, currentView.id)}
			onclick={() => openNewWindow(currentView.id)}
			class="flex w-full cursor-pointer flex-col items-center py-2 hover:bg-neutral-600"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="M1536 256h384v384h-128V475l-456 456l-91-91l456-456h-165zm0 768l128-128v768H0V512h1280l-128 128H128v896h1408z"
				/></svg
			>
		</button>
	</div>
</aside>
