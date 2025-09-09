import {Stack, Typography} from "@mui/joy";

export default function LogList({
    log
} : {
    log: string[]
}) {
    return (
        <Stack
            alignItems="center"
            justifyContent="right"
            sx={{width: '100%'}}
            gap={1}
        >
            {log.map((line, index) => (
                <Typography key={index} color="neutral" level="body-xs">
                    {line}
                </Typography>
            ))}
        </Stack>
    );
}
