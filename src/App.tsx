import {ThemeProvider} from "@mui/joy";
import theme from "./theme.ts";
import MainPage from "./MainPage.tsx";

function App() {
    return (
        <ThemeProvider theme={theme}>
            <MainPage/>
        </ThemeProvider>
    );
}

export default App;
