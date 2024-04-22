import { invoke } from "@tauri-apps/api/core";
import {
	createContext,
	createMemo,
	createResource,
	type InitializedResource,
	type ParentComponent,
	useContext,
} from "solid-js";
import type { CustomUserDetails } from "../../src-tauri/bindings/CustomUserDetails";
import { useTokenProvider } from "./TokenContext";

type UserContextData = [details: InitializedResource<CustomUserDetails>];

const UserContext = createContext<UserContextData>();

export const UserProvider: ParentComponent = (props) => {
	const [apiToken, apiReady] = useTokenProvider();
	const readyToRun = createMemo(() => apiReady() && apiToken().length > 0);
	const [details, { refetch }] = createResource<CustomUserDetails>(
		async () => {
			if (!apiReady() || apiToken().length === 0)
				return {
					email: "",
					organizations: [],
					id: "",
					suspended: false,
				};
			return (await invoke("get_user_details", {})) as CustomUserDetails;
		},
		{
			initialValue: {
				email: "",
				organizations: [],
				id: "",
				suspended: false,
			},
		},
	);

	createMemo(() => {
		if (readyToRun()) {
			refetch();
		}
	});

	return <UserContext.Provider value={[details]}>{props.children}</UserContext.Provider>;
};

export const useUserProvider = () => useContext(UserContext);
