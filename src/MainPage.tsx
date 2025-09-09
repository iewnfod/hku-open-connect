import React, {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";
import {
    Box,
    Button,
    Checkbox,
    Divider,
    Drawer,
    FormControl,
    FormLabel,
    IconButton,
    Input,
    Stack,
    Typography
} from "@mui/joy";
import LogList from "./LogList.tsx";
import TotpModal from "./TotpModal.tsx";
import DisconnectSnackBar from "./DisconnectSnackBar.tsx";
import LoginFailedSnackBar from "./LoginFailedSnackBar.tsx";
import RestartAltTwoToneIcon from '@mui/icons-material/RestartAltTwoTone';

const DEFAULT_HOST = "vpn2fa.hku.hk";

export default function MainPage() {
    const [open, setOpen] = useState(false);
    const [loading, setLoading] = useState(false);
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [usernameError, setUsernameError] = useState<boolean>(false);
    const [passwordError, setPasswordError] = useState<boolean>(false);
    const [showTotp, setShowTotp] = useState<boolean>(false);
    const [connected, setConnected] = useState<boolean>(localStorage.getItem("connected") === "true" || false);
    const [showDisconnect, setShowDisconnect] = useState<boolean>(false);
    const [showLoginFailed, setShowLoginFailed] = useState<boolean>(false);
    const [log, setLog] = useState<string[]>(["Welcome to HKU VPN (OpenConnect) Client!"]);
    const [showLog, setShowLog] = useState<boolean>(false);
    const [host, setHost] = useState<string>(localStorage.getItem("host") ?? DEFAULT_HOST);
    const [hostError, setHostError] = useState<boolean>(false);

    useEffect(() => {
        setTimeout(() => {
            setOpen(true);
        }, 100);
    }, []);

    useEffect(() => {
        localStorage.setItem("connected", JSON.stringify(connected));
    }, [connected]);

    useEffect(() => {
        listen("totp", () => {
            setShowTotp(true);
        });

        listen("connected", () => {
            setConnected(true);
            setLoading(false);
        });

        listen("login-failed", () => {
            setConnected(false);
            setLoading(false);
            setShowLoginFailed(true);
            setTimeout(() => {
                setShowLoginFailed(false);
            }, 5000);
        });

        listen("disconnect", () => {
            setConnected(false);
            setLoading(false);
            setShowDisconnect(true);
            setTimeout(() => {
                setShowDisconnect(false);
            }, 5000);
        });

        listen("user-disconnect", () => {
            setConnected(false);
            setLoading(false);
        });

        listen("add-log", (event) => {
            setLog((log) => {
                return [...log, event.payload as string];
            });
        });
    }, []);

    function handleDisconnect() {
        setLoading(true);
        invoke("disconnect_vpn");
    }

    function handleConnect() {
        if (connected) {
            handleDisconnect();
            return;
        }

        let flag = true;
        if (!username) {
            setUsernameError(true);
            flag = false;
        } else {
            setUsernameError(false);
        }

        if (!password) {
            setPasswordError(true);
            flag = false;
        } else {
            setPasswordError(false);
        }

        if (!host) {
            setHostError(true);
            flag = false;
        } else {
            localStorage.setItem("host", host);
            setHostError(false);
        }

        if (!flag) return;

        setShowTotp(false);
        setLoading(true);
        invoke("connect_vpn", {username, password, host});
    }

    async function handleTotpSubmit(totp: string) {
        await invoke("submit_totp", {totp});
    }

    return (
        <React.Fragment>
            <Box sx={{userSelect: 'none'}}>
                <Drawer open={open} size={showLog ? "lg" : "md"}>
                    <Stack direction="row" sx={{p: 5, height: '100%', overflowY: 'hidden'}} justifyContent="space-between" gap={5}>
                        <Stack sx={{height: '100%', width: '100%'}} gap={5}>
                            <Typography level="title-lg" sx={{userSelect: 'none'}}>
                                HKU VPN
                            </Typography>
                            <Stack justifyContent="space-between" sx={{height: '100%'}}>
                                <Stack gap={2}>
                                    <FormControl error={usernameError}>
                                        <FormLabel>
                                            Username
                                        </FormLabel>
                                        <Input
                                            placeholder="Email Address"
                                            value={username}
                                            onChange={e => setUsername(e.target.value)}
                                            disabled={connected || loading}
                                            sx={{userSelect: 'none'}}
                                        />
                                    </FormControl>
                                    <FormControl error={passwordError}>
                                        <FormLabel>
                                            Password
                                        </FormLabel>
                                        <Input
                                            placeholder="PIN"
                                            type="password"
                                            value={password}
                                            onChange={e => setPassword(e.target.value)}
                                            disabled={connected || loading}
                                            sx={{userSelect: 'none'}}
                                        />
                                    </FormControl>
                                    <FormControl error={hostError}>
                                        <FormLabel>
                                            Host
                                        </FormLabel>
                                        <Input
                                            type="url"
                                            value={host}
                                            onChange={e => setHost(e.target.value)}
                                            disabled={connected || loading}
                                            sx={{userSelect: 'none'}}
                                            endDecorator={(
                                                <IconButton
                                                    onClick={() => {
                                                        setHost(DEFAULT_HOST);
                                                        localStorage.setItem("host", DEFAULT_HOST);
                                                    }}
                                                >
                                                    <RestartAltTwoToneIcon/>
                                                </IconButton>
                                            )}
                                        />
                                    </FormControl>
                                </Stack>

                                <Stack justifyContent="center" alignItems="center" sx={{width: '100%'}} gap={2}>
                                    <Button
                                        loading={loading}
                                        onClick={() => handleConnect()}
                                        sx={{width: '100%'}}
                                    >
                                        {connected ? "Disconnect" : "Connect"}
                                    </Button>

                                    <Checkbox
                                        label="Show Log"
                                        checked={showLog}
                                        onChange={() => setShowLog(!showLog)}
                                    />
                                </Stack>
                            </Stack>
                        </Stack>

                        {
                            showLog ? (
                                <React.Fragment>
                                    <Divider orientation="vertical" />

                                    <Box sx={{minWidth: '40%', overflowY: 'auto', overflowX: 'hidden'}}>
                                        <LogList log={log} />
                                    </Box>
                                </React.Fragment>
                            ) : <></>
                        }
                    </Stack>
                </Drawer>

                <img
                    src="/login_bg.jpg"
                    alt="Background"
                    style={{
                        position: "absolute",
                        top: 0,
                        left: 0,
                        width: "100%",
                        height: "100%",
                        zIndex: -1,
                        objectFit: "cover"
                    }}
                />
            </Box>
            <TotpModal open={showTotp} onClose={() => setShowTotp(false)} onSubmit={handleTotpSubmit} />
            <DisconnectSnackBar open={showDisconnect} />
            <LoginFailedSnackBar open={showLoginFailed} />
        </React.Fragment>
    );
}