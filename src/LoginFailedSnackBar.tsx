import {Snackbar, Stack, Typography} from "@mui/joy";

import ErrorTwoToneIcon from '@mui/icons-material/ErrorTwoTone';

export default function LoginFailedSnackBar({
    open
} : {
    open: boolean
}) {
    return (
        <Snackbar open={open} color="danger">
            <Stack direction="column" gap={1}>
                <Stack direction="row" alignItems="center" gap={1} sx={{userSelect: 'none'}}>
                    <ErrorTwoToneIcon/>
                    <Typography level="body-lg">
                        Login Failed.
                    </Typography>
                </Stack>
                <Typography level="body-sm" color="neutral">
                    Your username, password or TOTP code might be wrong.
                </Typography>
            </Stack>
        </Snackbar>
    );
}
