<script lang="ts">
	let { children } = $props()
</script>

<div class="container mx-auto">
	<div class="grid grid-cols-2 gap-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
		{@render children()}
	</div>
</div>
