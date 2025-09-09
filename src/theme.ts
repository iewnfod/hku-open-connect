import {extendTheme} from "@mui/joy";

let theme = extendTheme({
    components: {
        JoyDrawer: {
            styleOverrides: {
                root: ({ ownerState }) => ({
                    ...(ownerState.size === 'lg' && {
                        '--Drawer-horizontalSize': 'clamp(600px, 75%, 100%)'
                    })
                })
            }
        }
    }
});

export default theme;
