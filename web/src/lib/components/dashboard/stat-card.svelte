<script lang="ts">
	import * as Card from "$lib/components/ui/card";

	let { title, value }: { title: string; value: string | number } = $props();
</script>

<Card.Root>
	<Card.Header>
		<Card.Title class="text-sm font-medium text-muted-foreground">{title}</Card.Title>
	</Card.Header>
	<Card.Content>
		<p class="text-2xl font-bold">{value}</p>
	</Card.Content>
</Card.Root>
