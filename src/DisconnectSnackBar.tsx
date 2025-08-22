import {Snackbar, Stack, Typography} from "@mui/joy";

import ErrorTwoToneIcon from '@mui/icons-material/ErrorTwoTone';

export default function DisconnectSnackBar({
    open
} : {
    open: boolean
}) {
    return (
        <Snackbar open={open} color="danger">
            <Stack direction="row" alignItems="center" gap={1} sx={{userSelect: 'none'}}>
                <ErrorTwoToneIcon/>
                <Typography>
                    Disconnected from VPN
                </Typography>
            </Stack>
        </Snackbar>
    )
}
