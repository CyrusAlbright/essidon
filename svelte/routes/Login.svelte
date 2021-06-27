<script>
	import { onDestroy, onMount } from "svelte";

	import { navbarVisible } from "../components/stores.js";

	let showPassword = false;

	onMount(() => {
		navbarVisible.update(_ => false)
	});

	onDestroy(() => {
		navbarVisible.update(_ => true);
	});
</script>

<svelte:head>
	<title>Login</title>
</svelte:head>

<main>
	<a href="/#/" id="close" aria-label="Close">
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1 -1 11 11">
			<path stroke-linecap="round" stroke-width="1.5" d="m0 0 9,9 M0 9 9,0" />
		</svg>
	</a>
	<section id="login">
		<fieldset>
			<legend>Login</legend>
			<div>
				<label for="username">Username</label>
				<input type="text" id="username" name="username">
			</div>
			<div>
				<label for="password">Password</label>
				<input type={showPassword ? "text" : "password"} id="password" name="password">
				<input type="checkbox" id="show-password" name="show-password" bind:checked={showPassword}>
				<label class="inline" for="show-password">Show password</label>
			</div>
			<button>
				Submit
			</button>
			<a href="/#/">Forgot password?</a>
		</fieldset>
		<p>
			Don't have an account?
			<a href="/#/">Sign up</a>
		</p>
	</section>
	<section id="background">

	</section>
</main>

<style>
	main {
		flex-grow: 1;
		
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: stretch;
		font-size: 1.2rem;
	}
	a#close {
		position: absolute;
		top: 0;
		left: 0;
		width: 30px;
		height: 30px;
		padding: 10px;
		stroke: #222;
		border-radius: 5px;
	}
	a#close:hover,
	a#close:focus {
		stroke: var(--color-2);
	}
	a#close:focus {
		outline: 1px dotted var(--color-2);
		outline-offset: -5px;
	}
	section {
		flex: 1 1 300px;
		transition: flex-basis 200ms, width 200ms;
	}
	section#login {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}
	section#login>* {
		margin: 10px 0;
	}
	fieldset {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;

		background-color: white;

		padding: 10px 20px;
		border-radius: 20px;
	}
	fieldset>* {
		margin: 10px 0;
	}
	section#background {
		background-image: var(--gradient-background);
	}
	legend {
		font-family: var(--header-font);
		font-variant: small-caps;
		font-size: 2rem;
		font-weight: bold;
		margin: 0 auto;
	}
	input[type="text"],
	input[type="password"] {
		display: block;
		font-size: 100%;
		font-family: inherit;
		appearance: none;
		text-decoration: none;
		border: 2px solid grey;
		border-radius: 100px;
		padding: 5px 10px;
		outline: none;
		transition: border-color 200ms;
	}
	input[type="text"]:hover,
	input[type="text"]:focus,
	input[type="password"]:hover,
	input[type="password"]:focus {
		border-color: var(--color-2);
	}
	input[type="text"]:focus,
	input[type="password"]:focus {
		outline: 1px solid var(--color-2);
		outline-offset: 2px;
	}
	label {
		margin-left: 10px;
	}
	label.inline {
		margin-left: 0;
		font-size: 1rem;
	}
	input[type="checkbox"] {
		appearance: none;
		position: relative;
		outline: none;
		cursor: pointer;
		top: 5px;
		height: 15px;
		width: 15px;
		vertical-align: top;
		display: inline-block;
		border: 1px solid grey;
		border-radius: 4px;
		background-color: white;
	}
	input[type="checkbox"]:focus {
		outline: 1px solid var(--color-2);
		outline-offset: 1px;
		border-color: var(--color-2);
	}
	input[type="checkbox"]:checked {
		background-color: var(--color-1);
		border-color: var(--color-2);
	}
	input[type="checkbox"]::after {
		display: block;
		position: absolute;
		top: 1px;
		left: 3px;
		content: "";
		width: 4px;
		height: 7px;
		border: 2px solid transparent;
		border-top: 0;
		border-left: 0;
		transform: rotate(35deg);
	}
	input[type="checkbox"]:checked::after {
		border-color: white;
	}
	input[type="checkbox"] + label {
		cursor: pointer;
	}
	button {
		--button-color: var(--color-1);
		font-size: 100%;
		font-family: inherit;
		text-decoration: none;
		appearance: none;
		border: none;

		padding: 5px 10px;
		border-radius: 100px;
		
		cursor: pointer;

		background-color: var(--button-color);
		color: white;

		transition: background-color 50ms, border-color 50ms, outline-offset 10ms;
	}
	button:focus {
		outline: 1px solid var(--button-color);
		outline-offset: 2px;
	}
	button:hover,
	button:focus {
		--button-color: var(--color-2);
	}
	button:active {
		--button-color: var(--color-1);
		outline-offset: 4px;
	}
	p, a {
		font-size: 1rem;
	}
	section a {
		--color: var(--color-1);
		color: var(--color);
		text-decoration: none;
	}
	section a:hover, a:active {
		--color: var(--color-2);
		text-decoration: underline;
	}
	section a:active, a:focus {
		outline: var(--color) dotted 1px;
		outline-offset: 2px;
	}
	@media screen and (max-width: 1000px) {
		section#background {
			flex-basis: 100px;
		}
	}
	@media screen and (max-width: 800px) {
		section#background {
			display: none;
		}
	}
</style>