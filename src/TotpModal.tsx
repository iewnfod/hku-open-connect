import {Button, DialogContent, DialogTitle, FormControl, FormLabel, Input, Modal, ModalDialog} from "@mui/joy";
import {useState} from "react";

export default function TotpModal({
    open,
    onClose,
    onSubmit
} : {
    open: boolean;
    onClose: () => void;
    onSubmit: (totp: string) => void;
}) {
    const [totp, setTotp] = useState('');
    const [totpError, setTotpError] = useState<boolean>(false);

    function handleSubmit() {
        if (totp.length === 0) {
            setTotpError(true);
            return;
        } else {
            setTotpError(false);
        }

        onSubmit(totp);
        setTotp('');
        onClose();
    }

    return (
        <Modal
            open={open}
            sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}
        >
            <ModalDialog>
                <DialogTitle>
                    TOTP
                </DialogTitle>
                <DialogContent>
                    Please enter the TOTP code from your authenticator app.
                </DialogContent>
                <FormControl error={totpError}>
                    <FormLabel>
                        TOTP Code
                    </FormLabel>
                    <Input
                        value={totp}
                        onChange={(e) => setTotp(e.target.value)}
                    />
                </FormControl>
                <Button onClick={handleSubmit}>
                    Submit
                </Button>
            </ModalDialog>
        </Modal>
    );
}
