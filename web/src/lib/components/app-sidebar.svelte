<script lang="ts">
	import { page } from "$app/state";
	import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
	import UsersIcon from "@lucide/svelte/icons/users";
	import ArrowRightLeftIcon from "@lucide/svelte/icons/arrow-right-left";
	import MegaphoneIcon from "@lucide/svelte/icons/megaphone";
	import SettingsIcon from "@lucide/svelte/icons/settings";
	import XIcon from "@lucide/svelte/icons/x";
	import ThemeSelect from "$lib/components/dashboard/theme-select.svelte";

	let navItems = [
		{ title: "Dashboard", href: "/", icon: LayoutDashboardIcon },
		{ title: "Affiliates", href: "/affiliates", icon: UsersIcon },
		{ title: "Campaigns", href: "/campaigns", icon: MegaphoneIcon },
		{ title: "Conversions", href: "/conversions", icon: ArrowRightLeftIcon },
	];

	let { open = $bindable(false) }: { open?: boolean } = $props();
</script>

{#if open}
	<button aria-label="Close sidebar" class="fixed inset-0 z-30 bg-black/50 md:hidden" onclick={() => (open = false)}></button>
{/if}

<aside class="fixed inset-y-0 left-0 z-40 flex w-60 flex-col bg-sidebar p-3 transition-transform duration-200 {open ? 'translate-x-0' : '-translate-x-full'} md:translate-x-0">
	<div class="flex flex-1 flex-col rounded-lg bg-sidebar-accent">
		<div class="flex h-14 items-center justify-between px-4">
			<a href="https://github.com/coltspy/affiliate-rs" target="_blank" rel="noopener noreferrer" class="text-base font-extrabold tracking-tight text-sidebar-accent-foreground transition-colors hover:text-primary">affiliate-rs</a>
			<button onclick={() => (open = false)} class="rounded-lg p-1 text-sidebar-foreground hover:text-sidebar-accent-foreground md:hidden">
				<XIcon class="h-5 w-5" />
			</button>
		</div>

		<div class="mx-4 border-t border-sidebar-border"></div>

		<nav class="flex-1 space-y-0.5 py-4">
			{#each navItems as item}
				{@const active = page.url.pathname === item.href}
				<a
					href={item.href}
					onclick={() => (open = false)}
					class="flex items-center gap-3 px-5 py-3 text-sm font-bold transition-colors {active ? 'bg-sidebar-border text-sidebar-accent-foreground' : 'text-sidebar-foreground hover:text-sidebar-accent-foreground'}"
				>
					<item.icon class="h-[18px] w-[18px]" />
					<span>{item.title}</span>
				</a>
			{/each}
		</nav>

		<div class="mx-4 border-t border-sidebar-border"></div>

		<div class="space-y-0.5 py-4">
			<a
				href="/settings"
				onclick={() => (open = false)}
				class="flex items-center gap-3 px-5 py-3 text-sm font-bold transition-colors {page.url.pathname.startsWith('/settings') ? 'bg-sidebar-border text-sidebar-accent-foreground' : 'text-sidebar-foreground hover:text-sidebar-accent-foreground'}"
			>
				<SettingsIcon class="h-[18px] w-[18px]" />
				<span>Settings</span>
			</a>
		</div>

		<div class="mx-4 border-t border-sidebar-border"></div>

		<div class="px-4 py-4">
			<ThemeSelect />
		</div>
	</div>
</aside>
