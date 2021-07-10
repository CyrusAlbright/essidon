<script>
	export let name;
	
	let input;

	export function getValue() {
		return input.value;
	}
</script>

<input {name} id={name}
	type="text" bind:this={input}>

<style>
	input[type="text"] {
		box-sizing: border-box;
		width: 250px;

		display: block;
		font-size: 100%;
		font-family: inherit;
		appearance: none;
		text-decoration: none;
		border: 2px solid grey;
		border-radius: 100px;
		padding: 5px 10px;
		outline: transparent;
		transition: border-color 200ms, outline-color 100ms;
	}
	input[type="text"]:hover,
	input[type="text"]:focus {
		border-color: var(--color-2);
	}
	input[type="text"]:focus {
		outline: 1px solid var(--color-2);
		outline-offset: 2px;
	}
</style>