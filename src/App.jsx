import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  return (
    <>
    <div class="container">
      <h1>Welcome to Tauri!</h1>
      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg()}</p>
    </div>
    <div>

<span class="loading loading-spinner loading-lg" />
    </div>
    <div class="carousel rounded-box">
  <div class="carousel-item">
    <img src="https://daisyui.com/images/stock/photo-1559703248-dcaaec9fab78.jpg" alt="Burger" />
  </div>
  <div class="carousel-item">
    <img src="https://daisyui.com/images/stock/photo-1565098772267-60af42b81ef2.jpg" alt="Burger" />
  </div>
  <div class="carousel-item">
    <img src="https://daisyui.com/images/stock/photo-1572635148818-ef6fd45eb394.jpg" alt="Burger" />
  </div>
  <div class="carousel-item">
    <img src="https://daisyui.com/images/stock/photo-1494253109108-2e30c049369b.jpg" alt="Burger" />
  </div>
  <div class="carousel-item">
    <img src="https://daisyui.com/images/stock/photo-1550258987-190a2d41a8ba.jpg" alt="Burger" />
  </div>
  <div class="carousel-item">
    <img src="https://daisyui.com/images/stock/photo-1559181567-c3190ca9959b.jpg" alt="Burger" />
  </div>
  <div class="carousel-item">
    <img src="https://daisyui.com/images/stock/photo-1601004890684-d8cbf643f5f2.jpg" alt="Burger" />
  </div>
</div>

  </>
  );
}

export default App;
