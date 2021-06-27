<svelte:options immutable={true}/>

<script>
	import { navbarVisible } from "./stores.js";

	export let active;
	export let links = [
		{
			title: "About",
			href: "#/about"
		},
		{
			title: "Contact",
			href: "#/contact"
		},
		{
			title: "Login",
			href: "#/login",
			style: "login"
		}
	];

	let visible;

	const unsubscribe = navbarVisible.subscribe(value => {
		visible = value;
	});

	function squish(node, { duration = 150 }) {
		let rect = node.getBoundingClientRect();

		let height = rect.bottom - rect.top;

		return {
			duration,
			css: (t, u) => {
				return `
					height: ${t*height}px;
				`;
			}
		};
	}

	function slide(node, { duration = 150 }) {
		return {
			duration,
			css: (t, u) => {
				return `
					transform: translateY(-${u * 100}%);
				`;
			}
		};
	}

	function slideOut(node, { duration = 150 }) {
		return {
			duration,
			css: (t, u) => {
				return `
					position: absolute;
					transform: translateY(-${u * 100}%);
					left: 0;
					right: 0;
				`;
			}
		};
	}
</script>

{#if visible}
	<header in:squish>
		<nav in:slide out:slideOut>
			<a class="logo" href="/#/">
				Essidon
			</a>
			<ul>
				{#each links as {title, href, style}}
					<li><a {href} class:active={active == href} class={style}>{title}</a></li>
				{/each}
			</ul>
		</nav>
	</header>
{/if}

<style>
	nav {
		position: relative;
		/* top: 0px; */
		background-color: var(--color-1);
		padding: 0 50px;
		line-height: 50px;
		box-shadow: 0 -5px 10px 5px rgba(0, 0, 0, 1);
		z-index: 1;
	}
	nav, ul {
		display: flex;
		justify-content: space-between;
		align-items: stretch;
	}
	a {
		text-align: center;
		text-decoration: none;
		color: white;

		transition: outline-width 50ms;
	}
	a:focus {
		outline: 3px solid var(--color-2);
		outline-offset: 0;
	}
	a.logo {
		font-family: var(--header-font);
		font-size: 2rem;
		font-variant: small-caps;
		font-weight: bold;
	}
	ul {
		list-style: none;
	}
	li {
		margin-left: 20px;
		display: flex;
		align-items: stretch;
		justify-content: center;
	}
	li a {
		font-size: 1.2rem;
		position: relative;
		padding: 0 20px;
	}
	li a.login {
		background-color: var(--color-3);
	}
	li a::after {
		content: "";
		position: absolute;
		bottom: 8px;
		left: 45%;
		width: 10%;
		border-bottom: 3px solid;
		border-color: rgba(255, 255, 255, 0);
		transition: all 0.2s ease;
	}
	li a.active::after, li a:hover::after, li a:focus::after {
		left: 10px;
		width: calc(100% - 20px);
		border-color: rgba(255, 255, 255, 1);
	}
	@media screen and (max-width: 800px) {
		nav {
			padding: 0;
		}
		nav, ul {
			flex-direction: column;
			align-items: stretch;
		}
		li {
			margin-left: 0;
		}
		a {
			width: 100%;
		}
		li a.active::after, li a:hover::after, li a:focus::after {
			left: calc(50% - 75px);
			width: 150px;
		}
	}
</style>