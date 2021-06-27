<script>

</script>

<svelte:head>
	<title>Home</title>
</svelte:head>

<main>
	<div>
		<section id="text">
			<h1>Welcome</h1>
			<p>
				Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
				Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
				Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
			</p>
		</section>
		<section id="image">
			<div></div>
		</section>
	</div>
</main>

<style>
	main {
		flex-grow: 1;
		background-image: var(--gradient-background);
	}
	main>div {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: stretch;

		margin: 20px 200px;
	}
	h1 {
		font-size: 2.5rem;
		margin-bottom: 20px;
	}
	p {
		font-size: 1.5rem;
	}
	section {
		flex: 1 1 200px;
		margin: 20px;
		color: white;

		display: flex;
		flex-direction: column;
		justify-content: center;
		
		min-height: 400px;
	}
	section#text {
		text-align: right;
	}
	/* section#image {

	} */
	section>div {
		width: 300px;
		height: 300px;

		border-radius: 50%;
		background-color: white;
	}
	@media screen and (max-width: 1200px) {
		main>div {
			margin: 20px 40px;
		}
		section#text {
			flex-basis: 300px;
		}
	}
	@media screen and (max-width: 800px) {
		main>div {
			flex-direction: column;
			margin: 20px 20px;
		}
		section {
			margin: 10px 0;

			min-height: inherit;
		}
		section#text {
			text-align: left;
		}
		section#text>p {
			text-indent: 1em;
		}
		section#image {
			align-items: center;
		}
	}
</style>