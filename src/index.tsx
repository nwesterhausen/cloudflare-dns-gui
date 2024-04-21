/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import App from "./App";
import { ZoneProvider } from "./ZoneProvider";
import { TokenProvider } from "./TokenProvider";

render(
	() => (
		<TokenProvider>
			<ZoneProvider>
				<App />
			</ZoneProvider>
		</TokenProvider>
	),
	document.getElementById("root") as HTMLElement,
);
