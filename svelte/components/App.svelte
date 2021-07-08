<script>
	import routie from "../lib/routie.min.js";

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

	/*function track(obj) {
		uri = obj.state || obj.uri || location.pathname;
	}

	addEventListener("replacestate", track);
	addEventListener("pushstate", track);
	addEventListener("popstate", track);*/

	routie({
		"/about": () => run(import("../routes/About.svelte")),
		"/contact": () => run(import("../routes/Contact.svelte")),
		"/login": () => run(import("../routes/Login.svelte")),
		"*": () => run(import("../routes/Home.svelte"))
	});
</script>

<Navbar {active}/>
<svelte:component this={Route} {params}/>