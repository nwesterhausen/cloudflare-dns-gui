/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import App from "./App";
import { ZoneProvider } from "./providers/ZoneContext";
import { TokenProvider } from "./providers/TokenContext";
import { UserProvider } from "./providers/UserContext";

render(
	() => (
		<TokenProvider>
			<UserProvider>
				<ZoneProvider>
					<App />
				</ZoneProvider>
			</UserProvider>
		</TokenProvider>
	),
	document.getElementById("root") as HTMLElement,
);
