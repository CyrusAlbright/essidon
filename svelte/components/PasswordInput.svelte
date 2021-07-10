<script>
	export let name = "password";
	
	let input;

	let inputFocused = false;
	let iconFocused = false;
	let checked = false;

	$: eitherFocused = inputFocused || iconFocused;
	$: if (!eitherFocused && checked) {
		checked = false;
	}

	export function getValue() {
		return input.value;
	}
</script>

<div>
	<input {name} id={name} bind:this={input}
		type={checked ? "text" : "password"}
		on:focus={() => inputFocused = true}
		on:blur={() => setTimeout(() => inputFocused = false, 0)}>
	<button aria-label="Toggle password visibility"
		class:hidden={!eitherFocused} class:checked={checked}
		on:click={() => { checked = !checked; iconFocused = true; input.focus(); }}
		on:focus={() => iconFocused = true}
		on:blur={() => setTimeout(() => iconFocused = false, 0)}>
		<svg viewBox="-50 -50 100 100" xmlns="http://www.w3.org/2000/svg">
			<path d="M -40 0 A 50 50 0 0 1 40 0" fill="none" stroke-width="10"/>
			<circle cx="0" cy="0" r="20" stroke-width="10" />
		</svg>
	</button>
</div>

<style>
	div {
		display: block;
		position: relative;
		box-sizing: border-box;
		width: 100%;
	}
	input {
		box-sizing: border-box;
		width: 250px;
		font-size: 100%;
		font-family: inherit;
		appearance: none;
		text-decoration: none;
		border: 2px solid grey;
		border-radius: 100px;
		padding: 5px 35px 5px 10px;
		outline: transparent;
		transition: border-color 200ms, outline-color 100ms;
	}
	input:hover,
	input:focus {
		border-color: var(--color-2);
	}
	input:focus {
		outline: 1px solid var(--color-2);
		outline-offset: 2px;
	}
	svg {
		display: block;
	}
	button {
		appearance: none;
		background: none;
		border: none;
		width: 20px;
		height: 20px;
		border-radius: 10px;
		position: absolute;
		right: 10px;
		top: 50%;
		transform: translateY(-50%);
		stroke: grey;
		fill: none;
		outline: transparent;
		cursor: pointer;

		transition: fill 100ms, stroke 100ms;
	}
	button.checked {
		fill: grey;
	}
	button:focus {
		outline: 1px solid var(--color-2);
		outline-offset: 2px;
	}
	button.checked:focus, button.checked:hover {
		fill: var(--color-2);
	}
	button:hover, button:focus {
		stroke: var(--color-2);
	}
	button.hidden {
		display: none;
	}
	::-ms-reveal, ::-ms-clear {
		display: none;
	}
</style>