<script>
	import { onDestroy } from "svelte";
	import Navaid from "navaid";

	import Navbar from "./Navbar.svelte";

	let Route;
	let params = {};

	let active;
	let uri = location.pathname;
	$: active = uri || "/";

	function run(thunk, obj) {
		const target = uri;

		thunk.then(m => {
			if (target !== uri) {
				return;
			}

			params = obj || null;

			if (m.preload) {
				m.preload({ params }).then(() => {
					if (target !== uri) {
						return;
					}

					Route = m.default;
					window.scrollTo(0, 0);
				});
			} else {
				Route = m.default;
				window.scrollTo(0, 0);
			}
		});
	}

	function track(obj) {
		uri = obj.state || obj.uri || location.pathname;
	}

	addEventListener("replacestate", track);
	addEventListener("pushstate", track);
	addEventListener("popstate", track);

	const router = Navaid("/")
		.on("/", () => run(import("../routes/Home.svelte")))
		.on("/about", () => run(import("../routes/About.svelte")))
		.on("/contact", () => run(import("../routes/Contact.svelte")))
		.on("/login", () => run(import("../routes/Login.svelte")))
		.listen();
	
	onDestroy(router.unlisten);
</script>

<Navbar {active}/>
<svelte:component this={Route} {params}/>